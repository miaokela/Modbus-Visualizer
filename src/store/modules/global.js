const global = {
  state: () => {
    return {
      layout: [
        { x: 0, y: 0, w: 2, h: 2, i: "d01feb1a9d8e42d1ba6c0577ed46ba26", static: false },
        { x: 4, y: 0, w: 2, h: 5, i: "aa6928cb77f8431b84d35fae57957658", static: false },
        { x: 6, y: 0, w: 2, h: 3, i: "1585465995a24e0fb1ca588e2ddfd10c", static: false },
        { x: 8, y: 0, w: 2, h: 3, i: "7cdc58cb1de2455f91dcd58592639678", static: false },
      ],
    };
  },
  mutations: {
    UPDATE_LAYOUT(state, idx, layout) {
      const index = state.layout.findIndex((item) => item.idx === idx);
      state.layout[index] = layout;
    },
    ADD_LAYOUT(state, layout) {
      state.layout.push(layout);
    },
    REMOVE_LAYOUT(state, idx) {
      const index = state.layout.findIndex((item) => item.i === idx);
      state.layout.splice(index, 1);
    }
  },
  actions: {
    updateLayout({ commit, idx, layout }) {
      commit("UPDATE_LAYOUT", idx, layout);
    },
    addLayout({ commit }, layout) {
      commit("ADD_LAYOUT", layout);
    },
    removeLayout({ commit }, idx) {
      commit("REMOVE_LAYOUT", idx);
    }
  },
  getters: {
    layout(state) {
      return state.layout;
    },
  },
};

export default global;
