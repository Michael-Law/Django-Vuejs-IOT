<template>
  <div class="signup">
    <navbar />
    <b-jumbotron
      header="Logistics Solutions for entreprise"
      lead="Choose the appropriate solution for your company or use the tierI option for a demo"
    >
    </b-jumbotron>
    <v-card style="max-width: 100rem" class="mb-2">
      <b-card-group deck>
        <b-card title="Tier I">
          <b-card-text>
            This tier your company will benefit the simple rerouting of the your
            vehicles based on public data.
          </b-card-text>
          <template #footer>
            <b-button variant="outline-primary" to="/tierI"
              >Contact Sales</b-button
            >
          </template>
        </b-card>

        <b-card title="Tier II">
          <b-card-text>
            Our technician will install custom made IOT devices specifically
            designed for your company needs, which will aggregate data for your
            specific needs. Then doing an non-generic optimisation.
          </b-card-text>
          <template #footer>
            <b-button variant="outline-primary" to="/tierII"
              >Contact Sales</b-button
            >
          </template>
        </b-card>

        <b-card title="Tier III">
          <b-card-text>
            Using blockchain verify your transactions and tag along your
            different data points by adding to our blockchain infracstructure.
            Thus you could be sure there will be no amering of data and seamless
            automatic transactions with your stakeholders
          </b-card-text>
          <template #footer>
            <b-button variant="outline-primary" to="/tierIII"
              >Contact Sales</b-button
            >
          </template>
        </b-card>
      </b-card-group>
    </v-card>
  </div>
</template>
<script>
import navbar from "@/components/navbar.vue";
import register from "@/components/register.vue";
export default {
  name: "Home",
  components: {
    navbar,
    register,
  },
};
</script>