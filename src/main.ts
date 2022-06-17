import Vue from 'vue'
import App from './App.vue'
import { constants } from './lib.rs'
import fluster from './fluster'

const state = {
}

const update = (name, value) => {
  state[name] = value
}

Vue.use(fluster, {
  state,
  update,
})

new Vue({
  render: h => h(App),
}).$mount('#app')