<template>
  <div id="app">
    <div id="nav">
      <top-navbar></top-navbar>
    </div>
    <router-view/>
  </div>
</template>
<script>
import Top from '@/components/navbar.vue'
export default {
  components:{
    'top-navbar':Top
  },
}
</script>
<style lang='scss'>
@import './components/forestgrey.css';
@import url("https://fonts.googleapis.com/css2?family=Arvo&display=swap");

#app {
  font-family: Avenir, Helvetica, Arvo, serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color:#c1c8c9;
  background: #feffff;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #2c3e50;
}

#nav a.router-link-exact-active {
  color: #42b983;
}
</style>
