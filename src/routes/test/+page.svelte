<!--Enable COOP and COEP headers for rust wasm webworkes. Otherwise special headers need to be manually set.
https://github.com/gzuidhof/coi-serviceworker-->
<script lang="ts" src="../coi-serviceworker.js">
    import {onMount} from 'svelte';
    import Grid from './Grid.svelte'

    onMount(async () => {

    });
</script>

<div>
    <!--Display grid-->
    <Grid/>
</div>