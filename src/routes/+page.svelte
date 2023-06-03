<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import FinanceChart from '$lib/components/FinanceChart.svelte';

	let chartData = {
		labels: [],
		datasets: [
			{
				label: 'Breakdown by Category',
				data: []
			}
		]
	};
	async function getData() {
		try {
			let financeData = await invoke('get_data_by_categories');
			console.log(financeData);
			chartData.labels = [...financeData.category];
			chartData.datasets[0].data = [...financeData.total];
			console.log(chartData);
		} catch (e) {
			console.error(e);
		}
	}
	async function showData() {
		try {
			let data = await invoke('get_state');
			console.log(data);
		} catch (e) {
			console.error(e);
		}
	}
</script>

<h1>Finance Analysis app</h1>
<a href="/add-data">Add Data</a>
<h2>Data Analysis</h2>
<button on:click={showData}>Show Data</button>
<button on:click={getData}>Get Data</button>
<FinanceChart bind:data={chartData} />
