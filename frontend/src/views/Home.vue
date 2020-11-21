<template>
  <div class="home">
  <header>Welcome to a new way of waste collection</header>
      <img :src="image"/>
  </div>
</template>

<script>
// @ is an alias to /src
import image from "src/assets/pexels-felix-mittermeier-957024.jpg"

export default {
  name: 'Home',
  components: {
    data: function () {
            return {
                image: image
            }
        }
  }
}
</script>
