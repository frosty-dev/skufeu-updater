<script>
    import { writeTextFile, exists, removeFile, removeDir, readTextFile, createDir } from "@tauri-apps/api/fs";
    import { fetch } from "@tauri-apps/api/http";
    import { open } from "@tauri-apps/api/dialog";
     // @ts-ignore
    import { metadata } from "tauri-plugin-fs-extra-api";
    // @ts-ignore
    import { download } from "tauri-plugin-upload-api";
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from "svelte";
    import { UPDATES_LINK, MAIN_LINK } from '../storage.js';
    
    let EXEC_DIR = "";

    let CURRENT_TASK = "Нажми \"Проверить обновления\", чтобы синхронизировать моды и некоторые конфиги"
    let LOGS = ""

    let BUSY = false;

    async function fetch_version() {
        return fetch($UPDATES_LINK + '/.client_version').then(r => r.data)
    }

    function get_diff_changes(a1, a2) {
        return a2.filter(function(o) {
            return Object.keys(o).some(function(k) {
                return a1.every(function(o2) {
                    return !(k in o2) || (o2[k] != o[k]);
                });
            });
        });
    }

    async function append_log(data) {
        LOGS = data + "\n" + LOGS
    }

    async function update_content() {
        if(BUSY) {
            return;
        }
        BUSY = true;
        let actual_content = await fetch($UPDATES_LINK + '/hash_new.json').then(r => r.data)
        let files_to_remove = [];
        if(await exists(EXEC_DIR + "hashv1.json")) {
            files_to_remove = get_diff_changes(actual_content, JSON.parse(await readTextFile(EXEC_DIR + "hashv1.json")));
        }
        let files_to_update = [];
        for (const ac of actual_content.filter((file) => file.hash !== "")) {
            CURRENT_TASK = "ПРОВЕРЯЕМ: " + ac.file;
            append_log("CHECK: " + ac.file);
            if(await exists(EXEC_DIR + ac.file)) {
                let current_file_hash = await invoke("get_hash", {filePath: EXEC_DIR + ac.file})
                if(current_file_hash !== ac.hash) {
                    files_to_remove.push(ac)
                }
            } else {
                files_to_update.push(ac)
            }
        }
        console.log("FTU")
        console.log(files_to_update)
        console.log(files_to_remove)
        for (const removed of files_to_remove) {
            if(await exists(EXEC_DIR + removed.file)) {
                let md = await metadata(EXEC_DIR + removed.file)
                if(md.isDir) {
                    await removeDir(EXEC_DIR + removed.file, { recursive: true })
                } else {
                    await removeFile(EXEC_DIR + removed.file)
                }
                append_log("RM: " + removed.file)
            }
        }

        await download_content(files_to_update)

        CURRENT_TASK = "ОБНОВЛЕНО"
        append_log("UPDATE DONE, МОЖЕШЬ ЗАКРЫВАТЬ ЁПТА")
        BUSY = false;
        writeTextFile(EXEC_DIR + "hashv1.json", JSON.stringify(actual_content))
    }

    async function download_content(files_to_update) {
        for (const downloaded of files_to_update) {
            if(downloaded.hash === "") {
                continue
            }
            let receivedLength = 0;
            let checked_path = downloaded.file.replace(/[^\/]*$/, '');
            if(!await exists(EXEC_DIR + checked_path)) {
                console.log(checked_path)
                createDir(EXEC_DIR + downloaded.file.replace(/[^\/]*$/, ''), { recursive: true })
            }
            await download(
                $UPDATES_LINK + "/" + downloaded.file,
                EXEC_DIR + downloaded.file,
                (progress, total) => {
                    receivedLength += progress;
                    CURRENT_TASK = `Загружаем ${downloaded.file}, скачано ${receivedLength} из ${total} байт`
                },
                // @ts-ignore
                { "Content-Type": "application/octet-stream" },
            );
            append_log("DL: " + downloaded.file)
        };
    }

    /** @param {MouseEvent} event */
    async function select_modpack_dir(event) {
        // Open a selection dialog for directories
        const selected = await open({
            directory: true,
            multiple: true,
            defaultPath: EXEC_DIR,
        });
        if (Array.isArray(selected)) {
            EXEC_DIR = selected[0] + "\\";
        } else if (selected === null) {
            EXEC_DIR = selected + "\\";
        } else {
            EXEC_DIR = selected + "\\";
        }
    }

    onMount(async () => {
        EXEC_DIR = await invoke("get_exec_path").then(path => path.substring(0, path.length - 6))
    });

    //let client_version = fetch_version()
</script>
<span>
    <button on:click={update_content}>Проверить обновления</button>
</span>
<span>
    <button on:click={select_modpack_dir}>Изменить локацию сборки</button>
</span>

<p class="logs">{LOGS}</p>
<p class="status">{CURRENT_TASK}</p>