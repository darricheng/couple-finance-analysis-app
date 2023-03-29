<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let data;
	async function getData() {
		try {
			data = await invoke('get_state');
			console.log(data);
		} catch (e) {
			console.error(e);
		}
	}
</script>

<h1>Finance Analysis app</h1>
<a href="/add-data">Add Data</a>
<h2>Data Analysis</h2>
<button on:click={getData}>Get Data</button>
