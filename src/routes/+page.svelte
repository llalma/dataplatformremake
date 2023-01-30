<!--Enable COOP and COEP headers for rust wasm webworkes. Otherwise special headers need to be manually set.
https://github.com/gzuidhof/coi-serviceworker-->
<script src="coi-serviceworker.js">

    const redirects = {Home: "/", test: "/test"}

</script>

<h1>Welcome to SvelteKit</h1>

<!-- Create Nav bar -->
<nav>
    {#each Object.entries(redirects) as [key, value]}
        <a href={value}>{key}</a><br>
    {/each}
</nav>