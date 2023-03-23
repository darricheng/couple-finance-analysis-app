<script lang="ts">
	let name = '';
	let data = '';
	let lines = [];
	let headers: string[] = [];
	function processData() {
		// TODO: function should store the data somewhere for use later
		// Take the first row of data and display it for mapping to the required data columns
		lines = data.split(/\r?\n/);
		headers = lines[0].split(',');
	}
	function storeData() {
		// TODO
	}
</script>

<h1>Add Data</h1>
<form on:submit={processData}>
	<input placeholder="Enter Your Name" bind:value={name} />
	<p>Paste your data in csv format with headers</p>
	<textarea bind:value={data} />
	<button type="submit">Submit</button>
</form>

<!-- https://stackoverflow.com/a/30593806 -->
<!-- <p style="white-space: pre-line;">{data}</p> -->
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
			{#each headers as header}
				<tr>
					<td data-userHeader={header}>{header}</td>
					<td>
						<select>
							<option value="date">Date</option>
							<option value="category">Category</option>
							<option value="value">Value</option>
						</select>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<button on:click={storeData}>Confirm</button>
{/if}

<br />
<a href="/">Back home</a>
