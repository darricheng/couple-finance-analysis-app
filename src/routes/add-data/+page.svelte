<script lang="ts">
	let name = '';
	let data = '';
	let lines = [];
	let headers: String[] = [];
	function processData() {
		// TODO: function should store the data somewhere for use later
		// Take the first row of data and display it for mapping to the required data columns
		lines = data.split(/\r?\n/);
		headers = lines[0].split(',');
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
{#if headers.length !== 0}
	<h2>Map your columns to the required data columns</h2>
	<ul>
		{#each headers as header}
			<li>{header}</li>
		{/each}
	</ul>
{/if}

<a href="/">Back home</a>
