<script lang="ts">
    import {onMount} from "svelte";
    import init, {Grid, initThreadPool } from '../../../rustFunctions/grid/pkg/grid'
    import * as Comlink from "https://unpkg.com/comlink/dist/esm/comlink.mjs";
    import './wasm-worker'

    let data;
    let grid = [4,3];

    let obj;
    let worker_ready = false;

    //File uploaded to load data
    let upload_file;

    // Mount Rust WASM Function
    onMount(async () => {
        // await init().then(() => {
        //     // data = new Grid(grid[0], grid[1]);
        // });
        const worker = new Worker(new URL("./wasm-worker.js", import.meta.url), {type:"module"});
        obj = Comlink.wrap(worker);
        await obj.initThreads().then(() => {
            worker_ready = true
            console.log('Web worker ready')
        })

        // console.log(await obj.data.get_cell(0,0))


    });

    function set_cell(e, i, j){
        data.set_cell(i, j, e.target.innerText)
        data=data
    }

    function download_csv_file(){
        const fileName = "test.csv"

        const blob = new Blob([data.to_csv()], {type: 'text/plain'});
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.download = fileName;
        link.href = url;
        link.click();
        URL.revokeObjectURL(url);
    }

    function load_csv_file(event){

        const file = event.target[0].files[0]

        let reader = new FileReader();
        reader.readAsArrayBuffer(file);
        reader.onload =  (async (data1) => {
            console.log(await obj.data.load_csv(new Uint8Array(data1.target.result)))
            obj.data = obj.data
        });
    }

</script>

<!--Load CSV File-->
<form on:submit|preventDefault={() => load_csv_file(event)}>
    <label>File</label>
    <input type="file" accept="text/csv" bind:value={upload_file}>
    <button type="submit">Load File</button>
</form>

<button on:click={download_csv_file}>Save To CSV</button>

{#if worker_ready}

    <table style="table">
        <!--Insert data cells-->
        {#each Array(grid[0]) as _,i}
            <tr>

                {#each Array(grid[1]) as _,j}
                    <td contenteditable="true"
                        on:blur={(event) => set_cell(event, i,j)}>
                        {#await obj.data.get_cell(i,j)}
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

    tr{
        border: solid;
        border-width: 1px;
    }

    td{
        border: solid;
        border-width: 2px;
    }
</style>


