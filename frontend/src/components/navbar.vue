<template>
  <div>
    <v-toolbar>
      <v-toolbar-title class="AnotherStyle">
        <span router-link to="/">HygeaTech </span>
        <span class="mystyle">| Logistics Services</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>

      <v-row dense align="end" align-content="end">
        <v-spacer />
        <v-btn text> Sign in </v-btn>
        <v-btn text> Register </v-btn>
      </v-row>
    </v-toolbar>
  </div>
</template>

<script>
export default {};
</script>

<style lang="scss">
@import url("https://fonts.googleapis.com/css?family=Questrial");
@import url("https://fonts.googleapis.com/css2?family=Pacifico&display=swap");

.AnotherStyle {
  font-family: "Pacifico", cursive;
}
.mystyle {
  font-family: "Questrial";
}
</style>


