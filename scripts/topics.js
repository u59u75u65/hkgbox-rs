
function parseQueryString(s) {
    var query = (s || '?').substr(1),
        map   = {};
    query.replace(/([^&=]+)=?([^&]*)(?:&+|$)/g, function(match, key, value) {
        map[key] = value;
    });
    return map;
}

var result = $(".Topic_ListPanel tr[id]")
  .map(function() {
    t = $(this).find("td:not(:first)")
      .map(function(i, item) {
        switch (i) {
          case 0:
            return {
              titles: $(this).find('a').map(function() {
                var start = href.indexOf('?');
                return {
                  url: $(this).prop('href'),
                  url_query: parseQueryString(href.substring(start)),
                  text: $(this).text().trim()
                }
              }).toArray()
            };

          case 1:
            var a = $(this).find('a').first();
            if (a.size()) {
              return {
                url: a.prop('href'),
                name: a.text().trim()
              }
            } else {
              return {}
            }
          case 2:
            return $($(this).text().trim().split('\n')).map(function() {
              return this.trim();
            }).toArray();
          default:
            return $(this).text().trim();
        }

      }).toArray()
      .reduce(function(o, v, i) {
        // console.log(o);
        // console.log(v);
        // console.log(i);
        switch (i) {
          case 0:
            o["titles"] = v.titles;
            return o;
          case 1:
            o["author"] = v;
            return o;
          case 2:
            o["last_replied_date"] = v;
            return o;
          case 3:
            o["last_replied_time"] = v;
            return o;
          case 4:
            o["reply_count"] = v;
            return o;
          case 5:
            o["rating"] = v;
            return o;
        }
      }, {})

    t["id"] = this.id;
    return t;

  }).toArray();

  JSON.stringify(result);
