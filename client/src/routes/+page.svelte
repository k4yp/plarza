<script>
    /**
     * @type {any[]}
     */

    let posts = [];
  
    const loadPosts = async () => {
      const response = await fetch('http://localhost:5500/server/data.csv');
      const data = await response.text();
      const rows = data.split('\n');
  
      rows.forEach((row) => {
        const [user, source, date, caption, media, link] = row.split('•');
        posts.push({ user, source, date, caption, media, link });
      });
      posts = posts.slice(1);
    };
    
    import { onMount } from 'svelte';
  
    onMount(loadPosts);

    console.log(posts)
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
    border-top: 1px solid;
    border-left: 1px solid;
    border-right: 1px solid;
    border-color: rgb(217, 217, 217);
    margin: auto;
    padding: 16px;
    width: 30%;
}
a {
    display: block;
    text-decoration: none;
    color: inherit;
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
</style>