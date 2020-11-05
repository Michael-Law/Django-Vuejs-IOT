<template>
  <div>
    <v-toolbar class="pa-0">
      <v-toolbar-title class="AnotherStyle">
        <span router-link to="/">HygeaTech </span>
        <span class="mystyle">| Logistics Services</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>

      <v-row dense align="end" align-content="end">
        <v-spacer />
        <v-btn text to="/"> Home </v-btn>
        <v-btn text> Sign in </v-btn>
        <v-btn text> About </v-btn>
        <v-btn text> Solutions </v-btn>
      </v-row>
    </v-toolbar>
  </div>
</template>

<script>
export default {};
</script>

<style lang="scss">
@import url("https://fonts.googleapis.com/css2?family=Pathway+Gothic+One&display=swap");
@import url("https://fonts.googleapis.com/css2?family=Pacifico&display=swap");

.AnotherStyle {
  font-family: "Pacifico", cursive;
}
.mystyle {
  font-family: "Pathway Gothic One", sans-serif;
}

.v-toolbar__content {
  padding: 10px !important;
}
</style>
