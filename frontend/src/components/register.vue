<template>
  <div class="register">
    <b-card tag="article" style="max-width: 22.5rem" class="mb-2">
      <h1>{{ Tier }}</h1>
      <b-form @submit="onSubmit" @reset="onReset" v-if="show">
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

        <b-form-group
          id="input-group-2"
          label="Your first Name:"
          label-for="input-2"
        >
          <b-form-input
            id="input-2"
            v-model="form.username"
            required
            placeholder="Enter first name"
          ></b-form-input>
        </b-form-group>

        <b-form-group
          id="input-group-3"
          label="Your last Name:"
          label-for="input-3"
        >
          <b-form-input
            id="input-3"
            v-model="form.lastname"
            required
            placeholder="Enter last name"
          ></b-form-input>
        </b-form-group>

        <b-form-group
          id="input-group-4"
          label="Your Company's Name:"
          label-for="input-4"
        >
          <b-form-input
            id="input-4"
            v-model="form.company"
            required
            placeholder="Enter Company name"
          ></b-form-input>
        </b-form-group>

        <b-form-group
          id="input-group-5"
          label="Your company's size"
          label-for="input-5"
        >
          <b-form-select
            id="input-5"
            v-model="form.size"
            :options="size"
            required
          ></b-form-select>
        </b-form-group>

        <b-form-group id="input-group-1" label="Password:" label-for="input-6">
          <b-form-input
            id="input-6"
            v-model="form.password"
            type="password"
            required
            placeholder="Enter password"
          ></b-form-input>
        </b-form-group>

        <b-form-group
          id="input-group-1"
          label="Confirm Password again:"
          label-for="input-6"
        >
          <b-form-input
            id="input-6"
            v-model="form.password2"
            type="password"
            required
            placeholder="Enter password"
          ></b-form-input>
        </b-form-group>

        <b-button type="submit" variant="primary">Submit</b-button>
        <b-button type="reset" variant="danger">Reset</b-button>
      </b-form>
    </b-card>
  </div>
</template>

<script>
import axios from "axios";

export default {
  props: ["Tier"],
  data() {
    return {
      form: {
        username: "",
        email: "",
        name: "",
        company: "",
        size: "",
        password: "",
        password2: "",
        tier: this.Tier,
      },
      show: true,
      size: [
        { text: "Select One", value: null },
        "1-5",
        "5-20",
        "20-50",
        "50-100",
        "100-200",
      ],
      show: true,
    };
  },
  methods: {
    onSubmit(evt) {
      var form_data = new FormData();
      for (var key in this.form) {
        form_data.append(key, this.form[key]);
      }

      axios
        .post("http://127.0.0.1:8000/account/api/register", form_data)
        .then((response) => {
          const status = response.status;
          //redirect logic
          // var navigate = self.$router;
          if (status == "200") {
            router.push({ name: "dashboard" });
          }
        });
    },

    onReset(evt) {
      evt.preventDefault();
      // Reset our form values
      this.form.email = "";
      this.form.name = "";
      this.form.food = null;
      this.form.checked = [];
      // Trick to reset/clear native browser form validation state
      this.show = false;
      this.$nextTick(() => {
        this.show = true;
      });
    },
  },
};
</script>