<script>
    import { UPDATES_LINK } from '../storage.js';
    import { fetch, ResponseType } from "@tauri-apps/api/http";
    const CHANGELOG = fetch_news();
    async function fetch_news() {
        return await fetch($UPDATES_LINK + '/info.html',{method: "GET",responseType: ResponseType.Text}).then(r => r.data)
    }
</script>
<p class="changelog">
    {#await CHANGELOG}
        Загружаем список изменений...
    {:then CHANGELOG}
        {@html CHANGELOG}
    {:catch error}
        {error.message}
    {/await}
</p>