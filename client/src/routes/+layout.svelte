<script>
    import { inject } from '@vercel/analytics'
    import Navbar from "./Navbar.svelte";
    import Footer from "./Footer.svelte";

    inject({ mode: 'production' });
</script>

<Navbar />

<slot />

<Footer />