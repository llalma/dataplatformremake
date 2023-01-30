import init, {Grid, initThreadPool } from '../../../rustFunctions/grid/pkg/grid.js'
import * as Comlink from 'comlink'


const initThreads = {
    async initThreads() {
        this.data = await init().then(async () => {
            await initThreadPool(1)
            return new Grid(4, 3);
        })
    },
};

Comlink.expose(initThreads);