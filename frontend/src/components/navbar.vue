<template>
   <b-navbar toggleable="sm" type="light" variant="light">
    <b-navbar-toggle target="nav-text-collapse"></b-navbar-toggle>

    <b-navbar-brand>HygeiaTech.IO</b-navbar-brand>

    <b-collapse id="nav-text-collapse" is-nav>
      <b-navbar-nav>
        <b-nav-item to = '/'>Home</b-nav-item>
        <b-nav-item to = '/about'>Our mission</b-nav-item>
      </b-navbar-nav>
    </b-collapse>
  </b-navbar>
</template>

<script>
export default {
    
}
</script>

<style lang="scss">

#nav a.navbar-brand {
    font-family:  Arvo, serif;
    font-size: 10pt;
    color: blue; 
}
#nav a.nav-link {
    color: blue;    
}
</style>