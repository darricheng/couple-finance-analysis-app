<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	type Column = '' | 'date' | 'category' | 'amount';

	let name = '';
	let data = '';
	let headers: string[] = [];
	let mappedHeaders: Column[] = [];
	let validData: boolean;
	let endOfHeaders = 0;

	function processData() {
		endOfHeaders = data.search(/\r?\n/);
		headers = data.slice(0, endOfHeaders).split(',');
		mappedHeaders = new Array(headers.length);
		mappedHeaders.fill('');
	}
	function storeData() {
		let countObj = { date: 0, category: 0, amount: 0 };
		for (let value of mappedHeaders) {
			if (value !== '') countObj[value]++;
		}
		if (countObj.date !== 1 || countObj.category !== 1 || countObj.amount !== 1) {
			return (validData = false);
		}
		if (name === '') {
			return;
		}
		validData = true;
		let validCsv = mappedHeaders.join(',') + data.slice(endOfHeaders);
		invoke('parse_csv_to_state', { csvData: validCsv, name });
		// TODO: handle the returned Result to show relevant success/failure messages to the user
	}
</script>

<h1>Add Data</h1>
<form on:submit|preventDefault={processData}>
	<input placeholder="Enter Your Name" bind:value={name} />
	<p>Paste your data in csv format with headers</p>
	<textarea bind:value={data} />
	<button type="submit">Submit</button>
</form>

{#if headers.length >= 3}
	<h2>Map your columns to the required data columns</h2>
	<table>
		<thead>
			<tr>
				<th>Your Headers</th>
				<th>Mapped Headers</th>
			</tr>
		</thead>
		<tbody>
			{#each headers as header, i}
				<tr>
					<td data-userHeader={header}>{header}</td>
					<td>
						<select bind:value={mappedHeaders[i]}>
							<option value="" selected>None</option>
							<option value="date">Date</option>
							<option value="category">Category</option>
							<option value="amount">Amount</option>
						</select>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<button on:click={storeData}>Confirm</button>
{:else}
	<p>Your input data needs at least 3 columns to map to Date, Category, and Amount.</p>
{/if}

<p>
	{#if validData === false}
		Please ensure that the 3 given data columns are mapped to one header each
	{:else if validData === true}
		Data is valid!
	{/if}
</p>

{#if name === ''}
	<p>Please enter a name</p>
{/if}

<br />
<a href="/">Back home</a>
