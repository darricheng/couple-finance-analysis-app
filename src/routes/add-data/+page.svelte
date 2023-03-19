<script lang="ts">
	let data = '';
	function addData() {
		// TODO: function should store the data somewhere for use later
	}
</script>

<h1>Paste your csv data into the text box</h1>
<form on:submit={addData}>
	<textarea bind:value={data} />
	<button type="submit">Submit</button>
</form>

<!-- https://stackoverflow.com/a/30593806 -->
<p style="white-space: pre-line;">{data}</p>

<a href="/">Back home</a>
