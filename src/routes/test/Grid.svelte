<script lang="ts">
    import {onMount} from "svelte";
    import init, {Grid, initThreadPool } from '../../../rustFunctions/grid/pkg/grid'
    import * as Comlink from "https://unpkg.com/comlink/dist/esm/comlink.mjs";
    import './wasm-worker'

    let grid = [4,3];

    let inst;
    let worker_ready = false;

    // Mount Rust WASM Function
    onMount(async () => {
        // Import the remote workers class so it can be web workes can be initilised
        const MyClass = Comlink.wrap(new Worker(new URL("./wasm-worker.js", import.meta.url), {type:"module"}));
        inst = await new MyClass();
        await inst.init_workers().then(() => {
            worker_ready = true
        })
    });

    async function download_csv_file(){
        const fileName = "test.csv"

        const blob = new Blob([await inst.data.to_csv()], {type: 'text/plain'});
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.download = fileName;
        link.href = url;
        link.click();
        URL.revokeObjectURL(url);
    }

    function load_csv_file(file){
        let reader = new FileReader();
        reader.readAsArrayBuffer(file);
        reader.onload =  (async (data1) => {
            await inst.data.load_csv(new Uint8Array(data1.target.result))
                .then(async () => {
                    grid = await inst.data.get_shape()
                })
                .then(() => {
                    inst = inst;
                })
        });
    }

</script>

<!--Load CSV File-->
<form on:submit|preventDefault={() => load_csv_file(event.target[0].files[0])}>
    <label>File</label>
    <input type="file" accept="text/csv">
    <button type="submit">Load File</button>
</form>

<button on:click={download_csv_file}>Save To CSV</button>

{#if worker_ready}

    <table style="table">

        <!--Insert the column headers-->
        <tr>
            {#each Array(grid[1]) as _,i}
                <th contenteditable="true"
                    on:blur={(event) => inst.set_header(i, event.target.innerText).then(() => {inst=inst})}>
                    {#await inst.get_header(i)}
                    {:then val}
                        {val}
                    {/await}
                </th>
            {/each}
        </tr>

        <!--Insert data cells-->
        {#each Array(grid[0]) as _,i}
            <tr>

                {#each Array(grid[1]) as _,j}
                    <td contenteditable="true"
                        on:blur={(event) => inst.set_cell(i,j, event.target.innerText).then(() => {inst=inst})}>
                        {#await inst.get_cell(i,j)}
                            {:then val}
                            {val}
                        {/await}
                    </td>

                {/each}
            </tr>
        {/each}
    </table>

{/if}

<style>

    table{
        border-collapse: collapse;
    }

    th{
        border: solid;
        border-width: 2px;
    }

    td{
        border: solid;
        border-width: 2px;
    }
</style>


