Object.assign(window.search, {"doc_urls":["intro.html#introduction","core_game.html#core-game"],"index":{"documentStore":{"docInfo":{"0":{"body":54,"breadcrumbs":1,"title":1},"1":{"body":45,"breadcrumbs":2,"title":2}},"docs":{"0":{"body":"The goal of this tutorial series is not to teach you how to make a game in amethyst, but instead how to take a game you designed and make it ready to release. This tutorial is meant to bridge the gap between the Ping tutorial and making a game. I will be covering things like, Ui (and Ui Prefabs) Adding Multiple States Loading Screen Handling Settings Saving a game to a file Difficulty Levels To this end, I have made a simple snake game and the first chapter will briefly discuss the workings of the game. Which takes us to the next section.","breadcrumbs":"Introduction","id":"0","title":"Introduction"},"1":{"body":"screenshot In case everyone doesn't know, the mechanics of the game snake consist of: A snake that in controlled by the four directional keys (buttons will be added later) The snakes head moves around the field followed by the snakes body. If the snake hits the edge of the map or it's own tail it grows The map has food scattered randomly. If the snake hits the food, it grows longer and a new food is randomly placed.","breadcrumbs":"Core Game","id":"1","title":"Core Game"}},"length":2,"save":true},"fields":["title","body","breadcrumbs"],"index":{"body":{"root":{"a":{"d":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}}}}},"b":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":0,"docs":{},"w":{"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"o":{"d":{"df":0,"docs":{},"i":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"i":{"d":{"df":0,"docs":{},"g":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"c":{"a":{"df":0,"docs":{},"s":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"n":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}}},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"d":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"i":{"df":0,"docs":{},"f":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"l":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"df":0,"docs":{}}}},"r":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}},"s":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{},"e":{"d":{"df":0,"docs":{},"g":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"y":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}}},"f":{"df":0,"docs":{},"i":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}},"l":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}},"r":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"o":{"d":{"df":1,"docs":{"1":{"tf":1.7320508075688772}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"g":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":2.449489742783178},"1":{"tf":1.4142135623730951}}}},"p":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{},"o":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}},"h":{"a":{"df":0,"docs":{},"n":{"d":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"a":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"a":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"d":{"df":0,"docs":{},"u":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}},"t":{"'":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}},"k":{"df":0,"docs":{},"e":{"df":0,"docs":{},"y":{"df":1,"docs":{"1":{"tf":1.0}}}},"n":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"l":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"o":{"a":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"m":{"a":{"d":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}},"p":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}},"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"c":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"l":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}},"n":{"df":0,"docs":{},"e":{"df":0,"docs":{},"w":{"df":1,"docs":{"1":{"tf":1.0}}},"x":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"p":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":1,"docs":{"0":{"tf":1.0}}}}},"l":{"a":{"c":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"a":{"b":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"r":{"a":{"df":0,"docs":{},"n":{"d":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"a":{"d":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}},"s":{"a":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}},"c":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}},"s":{"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}},"t":{"df":1,"docs":{"0":{"tf":1.0}}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"n":{"a":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":2.449489742783178}}}}},"df":0,"docs":{}},"t":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"t":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}},"k":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}},"df":0,"docs":{},"e":{"a":{"c":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}},"h":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}}}}}},"u":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"w":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"breadcrumbs":{"root":{"a":{"d":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}}}}},"b":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":0,"docs":{},"w":{"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"o":{"d":{"df":0,"docs":{},"i":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"i":{"d":{"df":0,"docs":{},"g":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"c":{"a":{"df":0,"docs":{},"s":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"n":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}}},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"d":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"i":{"df":0,"docs":{},"f":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"l":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"df":0,"docs":{}}}},"r":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}},"s":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{},"e":{"d":{"df":0,"docs":{},"g":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"y":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}}},"f":{"df":0,"docs":{},"i":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}},"l":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}},"r":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"o":{"d":{"df":1,"docs":{"1":{"tf":1.7320508075688772}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"g":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":2.449489742783178},"1":{"tf":1.7320508075688772}}}},"p":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{},"o":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}},"h":{"a":{"df":0,"docs":{},"n":{"d":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"a":{"d":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"a":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"d":{"df":0,"docs":{},"u":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}},"t":{"'":{"df":1,"docs":{"1":{"tf":1.0}}},"df":0,"docs":{}}},"k":{"df":0,"docs":{},"e":{"df":0,"docs":{},"y":{"df":1,"docs":{"1":{"tf":1.0}}}},"n":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"l":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"o":{"a":{"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"m":{"a":{"d":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}},"p":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}},"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"c":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"l":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}},"n":{"df":0,"docs":{},"e":{"df":0,"docs":{},"w":{"df":1,"docs":{"1":{"tf":1.0}}},"x":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"p":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":1,"docs":{"0":{"tf":1.0}}}}},"l":{"a":{"c":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"a":{"b":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"r":{"a":{"df":0,"docs":{},"n":{"d":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{},"e":{"a":{"d":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}}}},"s":{"a":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}},"c":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"1":{"tf":1.0}}}}}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}},"s":{"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}},"t":{"df":1,"docs":{"0":{"tf":1.0}}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"n":{"a":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":2.449489742783178}}}}},"df":0,"docs":{}},"t":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"t":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}},"k":{"df":0,"docs":{},"e":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}},"df":0,"docs":{},"e":{"a":{"c":{"df":0,"docs":{},"h":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}},"h":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.7320508075688772}}}}}}}},"u":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}},"w":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"title":{"root":{"c":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{},"g":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{}},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"d":{"df":0,"docs":{},"u":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}}}}}},"pipeline":["trimmer","stopWordFilter","stemmer"],"ref":"id","version":"0.9.5"},"results_options":{"limit_results":30,"teaser_word_count":30},"search_options":{"bool":"OR","expand":true,"fields":{"body":{"boost":1},"breadcrumbs":{"boost":1},"title":{"boost":2}}}});