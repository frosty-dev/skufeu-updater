<script>
	import { appWindow } from '@tauri-apps/api/window'
	import ClientUpdater from './lib/ClientUpdater.svelte'
	import News from './lib/News.svelte'
	import { onMount } from "svelte";
	onMount(async () => {
		document
		.getElementById('titlebar-close')
		.addEventListener('click', () => appWindow.close())
	});

</script>
<div data-tauri-drag-region class="titlebar">
	SKUFEU UPDATER
	<div class="titlebar-button" id="titlebar-close">
		<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
	</div>
</div>
<main class="container">
	<div class="real-container">
		<div class="funny-buttons">
			<News/>
			<ClientUpdater/>
		</div>
	</div>
</main>

