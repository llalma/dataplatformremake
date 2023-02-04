import init, {Grid, initThreadPool } from '../../../rustFunctions/grid/pkg/grid.js'
import * as Comlink from 'comlink'


class MyClass{

    async init_workers() {
        await init()
        await initThreadPool(1)
        this.data = new Grid(4,3)
    };

    get_cell(i,j){
        return this.data.get_cell(i,j)
    }
    set_cell(i,j,data) {
        this.data.set_cell(i, j, data)
    }

    load_csv_file(file){
        let reader = new FileReader();
        reader.readAsArrayBuffer(file);
        reader.onload =  (async (data1) => {
            await this.data.load_csv(new Uint8Array(data1.target.result))
        });
    }

}
Comlink.expose(MyClass)