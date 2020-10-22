<template>
  <div>
    <b-navbar toggleable="sm" type="light" variant="light">
      <b-navbar-toggle target="nav-text-collapse"></b-navbar-toggle>

      <b-navbar-brand href="/">HygeiaTech.IO</b-navbar-brand>

      <b-collapse id="navbar-toggle-collapse" is-nav>
        <b-navbar-nav class="mr-auto">
          <b-nav-item href="#">Link 1</b-nav-item>
          <b-nav-item to="/about">About us</b-nav-item>
        </b-navbar-nav>
      </b-collapse>
    </b-navbar>
  </div>
</template>

<script>
export default {};
</script>

<style scoped>
#nav a.navbar-brand {
  font-family: "Pacifico", cursive;
  font-size: 25pt;
  color: #384552;
}

#nav a {
  font-family: "Roboto", sans-serif;
}
</style>