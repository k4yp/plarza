<script>
    import Navbar from "./Navbar.svelte";
    import Footer from "./Footer.svelte";

    import { inject } from '@vercel/analytics'
    import { injectSpeedInsights } from '@vercel/speed-insights/sveltekit';

    inject({ mode: 'production' });
    injectSpeedInsights();
</script>

<Navbar />

<slot />

<Footer />