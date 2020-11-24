<template>
  <div class="login">
    <b-card tag="article" style="max-width: 22.5rem" class="mb-2">
      <h2>Welcome Back</h2>
      <b-form @submit="onSubmit" v-if="show">
        <b-form-group
          id="input-group-1"
          label="Email address:"
          label-for="input-1"
          description="We'll never share your email with anyone else."
        >
          <b-form-input
            id="input-1"
            v-model="form.email"
            type="email"
            required
            placeholder="Enter email"
          ></b-form-input>
        </b-form-group>

        <b-form-group id="input-group-1" label="Password:" label-for="input-2">
          <b-form-input
            id="input-2"
            v-model="form.password"
            type="password"
            required
            placeholder="Enter password"
          ></b-form-input>
        </b-form-group>

        <b-button type="submit" variant="primary">Login</b-button>
      </b-form>
    </b-card>
  </div>
</template>

<script>
import axios from "axios";

export default {
  data() {
    type: "password";
    return {
      form: {
        email: "",
        password: "",
      },
      show: true,
    };
  },
  methods: {
    onSubmit(evt) {
      evt.preventDefault();
      alert(JSON.stringify(this.form));
      axios
        .post("http://127.0.0.1:8000/api/token/", this.form)
        .then((res) => {
          //Perform Success Action
        })
        .catch((error) => {
          // error.response.status Check status code
        })
        .finally(() => {
          //Perform action in always
        });
    },
  },
};
</script>