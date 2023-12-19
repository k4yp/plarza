<script>
    export let year = new Date().getFullYear();
</script>

<footer>
    <div class="row">
        <div class="column">
            <h3>Company</h3>
            <ul>
                <li><a href="/blog">Blog</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/careers">Careers</a></li>
                <li><a href="/contact">Contact</a></li>
            </ul>
        </div>
        <div class="column">
            <h3>Support</h3>
            <ul>
                <li><a href="/faq">FAQ</a></li>
                <li><a href="/docs">Documentation</a></li>
            </ul>
        </div>
        <div class="column">
            <h3>Legal</h3>
            <ul>
                <li><a href="/tos">Terms of Service</a></li>
                <li><a href="/privacy">Privacy Policy</a></li>
            </ul>
        </div>
        <div class="column">
            <h3>Follow Us</h3>
            <ul>
                <li><a href="https://x.com/plarzadotcom">X</a></li>
                <li><a href="https://youtube.com/@plarzadotcom">YouTube</a></li>
                <li><a href="https://instagram.com/plarza.com">Instagram</a></li>
                <li><a href="#">LinkedIn</a></li>
                <li><a href="#">Facebook</a></li>
            </ul>
        </div>
    </div>
</footer>

<style>
    footer {
        font-family: Inter, sans-serif;
        padding: 40px;
        position: fixed;
        bottom: 0;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .row {
        display: flex;
        justify-content: center;
        align-items: flex-start;;
        height: 100%;
    }

    .column {
        padding-left: 64px;
        padding-right: 64px;
        flex: 1;
    }

    ul {
        list-style-type: none;
        padding: 0;
    }

    a {
        color: black;
        text-decoration: none;
    }
</style>
