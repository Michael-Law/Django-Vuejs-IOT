import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import Register from '../views/signup.vue'
import Login from '../views/login.vue'
import dashboard from '../views/dashboard.vue'
import tierI from '../views/tierI.vue'
import tierII from '../views/tierII.vue'
import tierIII from '../views/tierIII.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/register',
    name: 'register',
    component: Register
  },
  {
    path: '/dashboard',
    name: 'dashboard',
    component: dashboard
  },
  {
    path: '/login',
    name: 'login',
    component: Login
  },
  {
    path: '/tierI',
    name: 'tierI',
    component: tierI
  },
  {
    path: '/tierII',
    name: 'tierII',
    component: tierII
  },
  {
    path: '/tierIII',
    name: 'tierIII',
    component: tierIII
  },
  {
    path: '/about',
    name: 'About',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
  }
]

const router = new VueRouter({
  routes
})

export default router
