import { createStore } from "vuex";
import greet from './modules/greet';

export default createStore({
    state: {},
    mutations: {},
    actions: {},
    getters: {},
    modules: {
        a: greet,
    },
});
