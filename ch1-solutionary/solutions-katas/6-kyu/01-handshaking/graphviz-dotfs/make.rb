dotfiles = Dir['*']

res = dotfiles.filter{ |pathname| !pathname.include?("make") }.map do |dotfile|
  [dotfile, dotfile.split('.')[0]]
end

res.each do |fname, outfname|
  cmd = 'dot.exe -Tpng -o ../images/' + outfname + '.png ' + fname
  system(cmd)
  puts('Creating ' + fname + ': ' + cmd)
end
