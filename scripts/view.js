
function parseQueryString() {
    var query = (window.location.search || '?').substr(1),
        map   = {};
    query.replace(/([^&=]+)=?([^&]*)(?:&+|$)/g, function(match, key, value) {
        map[key] = value;
    });
    return map;
}

topic_data = $(".repliers tr:first .repliers_header:last div").map(function(i){
  var s = $(this).text().trim();
  if (i == 1){
    var match = s.match(/^(\d+)個回應$/);
    return match ? match[1] : null;
  }
  return s;
})

replies_data = $(".repliers tr[userid][username]").map(function() {
  // arr = $(this).find(".repliers_right span.an-content-box").parent().text().trim().match(/^(\d{1,2}\/\d{1,2}\/\d{4}) (\d{1,2}\:\d{1,2}) \#(\d+)$/);
  var datetime = $(this).find(".repliers_right span:last").text().trim();
  return {
    userid: $(this).attr('userid'),
    username: $(this).attr('username'),
    content: $(this).find(".repliers_right .ContentGrid").first().html().trim(),
    published_at: datetime
  }
}).toArray();

var page = $("select[name='page']:last option:selected").text();
var max_page = $("select[name='page']:last option:last").text();
var result = {
  url_query: parseQueryString(window.location.search),
  title: topic_data.get(0),
  reply_count: topic_data.get(1),
  page: page,
  max_page: max_page,
  replies: replies_data,
};
JSON.stringify(result);
