<script>
    /**
     * @type {any[]}
     */
  
    let posts = [];

    const loadPosts = async () => {
        const response = await fetch('http://127.0.0.1:5000/posts')
        posts = await response.json()
    }
  
    import { onMount } from 'svelte';
  
    onMount(loadPosts);
  </script>
  
<div class="posts">
    {#each posts as post}
    <div class="post">
        <a href="{post.link}" class="post-header">
        <a href="https://{post.source}.com/{post.user}" class="user">@{post.user}@{post.source}.com</a>
        <p>{new Date(post.date * 1000).toLocaleString('en-GB')}</p>
        <img src={`https://t0.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=http://${post.source}.com&size=64`} class="source" alt={post.source}/>
        </a>
        <p class="caption">{post.caption}</p>
        {#if post.media !== 'None'}
        <img src={post.media} class="media" alt="{post.source} media"/>
        {/if}
    </div>
    {/each}
</div>

<style>
.posts {
    color: #272727;
    font-size: 16px;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}
.post {
    background-color: aliceblue;
    margin: auto;
    padding: 16px;
    margin-top: 16px;
    margin-bottom: 16px;
    border-radius: 16px;
    width: 35%;
    min-width: 500px;
}
a {
    display: block;
    text-decoration: none;
    color: inherit;
}
p{
    margin: 0;
}
.user:hover {
    text-decoration: underline;
}

.post-header {
    font-weight: bold;
    display: flex;
    align-items: center;
}
.source {
    padding-left: 10px;
    width: 16px;
}
.media {
    width: 100%;
    border-radius: 16px;
}
.user {
    padding-right: 10px;
}
.caption{
    padding-top: 16px;
    padding-bottom: 16px;
    margin: 0;
}
</style>