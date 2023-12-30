import { createRouter, createMemoryHistory } from 'vue-router'
import monitorRoutes from './modules/monitor';


const routes = [
  ...monitorRoutes,
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router
