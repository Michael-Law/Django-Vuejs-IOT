<template>
  <div>
  <b-navbar toggleable="sm" type="light" variant="light">
    <b-navbar-toggle target="nav-text-collapse"></b-navbar-toggle>
<b-navbar-nav>
    <b-navbar-brand>
       <b-link :to="'/'">HygeiaTech.IO</b-link>
    </b-navbar-brand>
</b-navbar-nav>
    <b-collapse id="nav-text-collapse" is-nav>
      <b-navbar-nav>
        <b-nav-text><b-link :to="'/about'">Our mission</b-link></b-nav-text>
      </b-navbar-nav>
    </b-collapse>
  </b-navbar>
  
</div>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  
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
