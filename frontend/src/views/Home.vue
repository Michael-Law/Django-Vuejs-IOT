<template>
  <div class="home">
    <navbar />

    <b-card
      overlay
      img-src="https://i.postimg.cc/jSpb4VjZ/chuttersnap-f-N603qc-EA7g-unsplash.jpg"
      img-alt="Card Image"
      text-variant="white"
      img-height="600"
      img-width="150"
    >
      <b-card-text>
        Powered by artificial intelligence and our mesh of internet of things
        devices, our software will simplify your scheduling, logistics planning
        and reduce drastically your costs, thus using our sophisticated
        intelligent nework, you can easily plan your course of action ahead.
      </b-card-text>
    </b-card>
    <b-card no-body class="overflow-hidden" style="max-width: 100%">
      <b-row no-gutters>
        <b-col md="6">
          <b-card-img
            src="https://i.postimg.cc/jSpb4VjZ/chuttersnap-f-N603qc-EA7g-unsplash.jpg"
            alt="Blabla"
            class="rounded-0"
            img-height="600"
            img-width="150"
          ></b-card-img>
        </b-col>
        <b-col md="6">
          <b-card-body title="Your logistics definitely Smarter">
            <p>
              Powered by artificial intelligence and our mesh of internet of
              things devices, our software will simplify your scheduling,
              logistics planning and reduce drastically your costs, thus using
              our sophisticated intelligent nework, you can easily plan your
              course of action ahead.
            </p>
          </b-card-body>
        </b-col>
      </b-row>
    </b-card>
    <h2>Logistics cases addressed; Garbage trucks retrieval and routing</h2>
    <p>
      With the continuing growth of our population and the increasing
      consumption of single items articles. Garbage retrieval is becoming a
      bigger task. More and more garbage is being produced per capita. So we, at
      Hygeia Technologies are embracing Internet of things to optimise your work
      flow, delivering the best data analystics.
    </p>
    <li>Artificial intelligence rerouting</li>
    <li>Advance data processing through the web</li>
    <li>Immediate update and self-actualisation of different data points</li>
  </div>
</template>

<script>
// @ is an alias to /src
import navbar from "@/components/navbar.vue";
export default {
  name: "Home",
  components: {
    navbar,
  },
};
</script>
<style lang="scss">
h1 {
  text-align: center;
}
</style>