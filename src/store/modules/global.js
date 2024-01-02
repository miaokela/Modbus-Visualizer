const global = {
  state: () => {
    return {
      layout: [
        // { x: 0, y: 0, w: 2, h: 2, i: "d01feb1a9d8e42d1ba6c0577ed46ba26", static: false },
        // { x: 4, y: 0, w: 2, h: 5, i: "aa6928cb77f8431b84d35fae57957658", static: false },
        // { x: 6, y: 0, w: 2, h: 3, i: "1585465995a24e0fb1ca588e2ddfd10c", static: false },
        // { x: 8, y: 0, w: 2, h: 3, i: "7cdc58cb1de2455f91dcd58592639678", static: false },
      ],
      gridState: false, // false 表示可编辑 true 表示采集 同时对应着 static
      resizeTag: 0,  // 使用时间戳 发生变化时重新画图
    };
  },
  mutations: {
    SET_RESIZE_TAG(state) {
      state.resizeTag = Date.now();
    },
    SET_GRID_STATE(state) {
      state.gridState = !state.gridState;
      // 如果gridState为0将layout的static全部设置成true，如果为1将layout的static全部设置成false
      let tmpLayout = [];
      console.log(state.gridState)
      if (state.gridState) {
        state.layout.forEach((item) => {
          tmpLayout.push({ ...item, static: true });
        });
      } else {
        state.layout.forEach((item) => {
          tmpLayout.push({ ...item, static: false });
        });
      }
      state.layout = tmpLayout;
      console.log(`layout: ${JSON.stringify(state.layout)}`);
    },
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
    setResizeTag({ commit }) {
      commit("SET_RESIZE_TAG");
    },
    setGridState({ commit }, gridState) {
      commit("SET_GRID_STATE", gridState);
    },
    updateLayout({ commit, idx, layout }) {
      commit("UPDATE_LAYOUT", idx, layout);
    },
    addLayout({ commit }, layout) {
      commit("ADD_LAYOUT", layout);
    },
    removeLayout({ commit }, idx) {
      commit("REMOVE_LAYOUT", idx);
    },
  },
  getters: {
    layout(state) {
      return state.layout;
    },
    gridState(state) {
      return state.gridState;
    },
    resizeTag(state) {
      return state.resizeTag;
    }
  },
};

export default global;
