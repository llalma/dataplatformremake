import init, {Grid, initThreadPool } from '../../../rustFunctions/grid/pkg/grid.js'
import * as Comlink from 'comlink'


class MyClass{

    grid = ''

    async init_workers() {
        await init()
        await initThreadPool(1)
        this.grid = new Grid(4,3)
    };

}

Comlink.expose(MyClass)