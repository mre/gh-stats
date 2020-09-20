| Repository | Stars |
| :--------- | ----: |
{% for repo in repos -%}
| [{{ repo.name }}]({{ repo.html_url | safe }}) | {{ repo.stargazers_count }} ★ |
{% endfor -%}
