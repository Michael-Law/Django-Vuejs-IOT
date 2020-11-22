<template>
  <div class="login">
    <navbar />
    <loginform />
  </div>
</template>
<script>
import loginform from "@/components/loginform.vue";
import navbar from "@/components/navbar.vue";

export default {
  name: "login",
  components: {
    loginform,
    navbar,
  },
};
</script>