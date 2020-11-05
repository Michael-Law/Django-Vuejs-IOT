<template>
  <div class="signup">
    <navbar />
    <v-card style="max-width: 50rem" class="mb-2">
      <v-card-title>Logistics Solutions for entreprise</v-card-title>
      <b-card-group deck>
        <v-slide-group>
          <b-card title="Tier I">
            <b-card-text>
              This tier your company will benefit the simple rerouting of the
              your vehicles based on public data.
            </b-card-text>
            <template #footer>
              <b-button variant="primary">Contact Sales</b-button>
            </template>
          </b-card>

          <b-card title="Tier II">
            <b-card-text>
              This card has supporting text below as a natural lead-in to
              additional content.
            </b-card-text>
            <template #footer>
              <b-button variant="primary">Contact Sales</b-button>
            </template>
          </b-card>

          <b-card title="Tier III">
            <b-card-text>
              This is a wider card with supporting text below as a natural
              lead-in to additional content. This card has even longer content
              than the first to show that equal height action.
            </b-card-text>
            <template #footer>
              <b-button variant="primary">Contact Sales</b-button>
            </template>
          </b-card>
        </v-slide-group>
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