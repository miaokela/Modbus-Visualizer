import { createStore } from "vuex";
import greet from './modules/greet';
import global from './modules/global';

export default createStore({
    state: {},
    mutations: {},
    actions: {},
    getters: {},
    modules: {
        greet: greet,
        global: global
    },
});
