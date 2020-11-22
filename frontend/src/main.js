import '@babel/polyfill'
import 'mutationobserver-shim'
import Vue from 'vue'
import './plugins/bootstrap-vue'
import App from './App.vue'
import router from './router'
import store from './store'
import vuetify from './plugins/vuetify';
import axios from 'axios'
import VueAxios from 'vue-axios'
import jwt from 'jwt-decode'
import vuex from 'vuex'

Vue.config.productionTip = false

new Vue({
  router,
  store,
  vuetify,
  axios,
  VueAxios,
  jwt,
  vuex,
  render: h => h(App)
}).$mount('#app')
