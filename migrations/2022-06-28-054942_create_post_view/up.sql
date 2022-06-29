CREATE VIEW post_view AS
SELECT id,
       title,
       short_content,
       slug,
       commentaries_open,
       (select array_agg((tags.id, name, tags.slug)::tag)
        from tags
        where tags.slug in (select post_tags_pivot.tag_slug from post_tags_pivot where post_slug = posts.slug)
        limit 5)                                                                   as "tags",
       (select (posts.title, posts.slug)::nav_post from posts where id < posts.id) as "prev",
       (select (posts.title, posts.slug)::nav_post from posts where id > posts.id) as "next",
       posts.updated_at
from posts;