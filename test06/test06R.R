
# 
# test06R.R
# 
# rust/test06/src/main.rsの検証用。
# Beta分布のCDFの逆関数を出す。
# そしてRustとC++/Boostの結果とともにグラフにする。
# 

library( dplyr)
library( tidyverse)

# File system 
getwd()
setwd( "c:/data")

# データを読む
df_rust = utils::read.csv(
  "test06out.csv", fileEncoding = "UTF-8"
)
df_cpp = read.csv(
  "test06cppout.csv", fileEncoding = "UTF-8"
)

# マージ用の識別コード
df_rust$code = df_rust$alpha * 10 * 10000 + df_rust$beta * 10 * 100 + df_rust$p * 100
df_cpp$code = df_cpp$alpha * 10 * 10000 + df_cpp$beta * 10 * 100 + df_cpp$p * 100

# データをマージ
df_cpp_tomerge = df_cpp[ , c( "code", "z_hat_cpp")]
df_all = right_join( df_rust, df_cpp_tomerge, by = c( "code" = "code"))

# Rで算出されるBetaのInvCDFを追加
df_all$z_hat_r = qbeta( df_all$p, df_all$alpha, df_all$beta)

# C++/Boostを別変数としても置いておく
df_all$cppvalue = df_all$z_hat_cpp

# データをlongにする
df_long = gather(
  df_all, key = "method", value = "z_hat", z_hat_1, z_hat_2, z_hat_cpp, z_hat_r
)
df_long$method[ df_long$method == "z_hat_1"] = "Rust/statr"
df_long$method[ df_long$method == "z_hat_2"] = "Rust/probability"
df_long$method[ df_long$method == "z_hat_cpp"] = "C++/Boost"
df_long$method[ df_long$method == "z_hat_r"] = "R"

# C++/Boostを真値と仮定した場合の差分（符号あり）
df_long$diff = df_long$z_hat - df_long$cppvalue

# C++/Boostを真値と仮定した場合の相対誤差
df_long$relerr = abs( df_long$z_hat - df_long$cppvalue) / df_long$cppvalue


## グラフでInvCDFを確認
## →見た目ではズレがわからない。

# グラフのパネル名にあたる文字列をつくる
df_long$label = paste( "alpha=", df_long$alpha, ", beta=", df_long$beta, sep = "")

# とにかくpとInvCDFでプロット

plotobj = ggplot( df_long, aes( x = p, y = z_hat)) +
  labs( x = "p") +
  labs( y = "InvCDF") +
  labs( title = "Beta InvCDF by 4 Methods") +
  theme( plot.title = element_text( hjust = 0.5)) +
  geom_point( aes( color = method), size = 1) +
  facet_wrap( ~ label, ncol = 3) 

print( plotobj)

outfilename = "test06R"

ggsave( file = paste( outfilename, "01.png", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)
ggsave( file = paste( outfilename, "01.svg", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)

# 同じものをジッタリングする

plotobj = ggplot( df_long, aes( x = p, y = z_hat)) +
  labs( x = "p") +
  labs( y = "InvCDF") +
  theme( plot.title = element_text( hjust = 0.5)) +
  geom_jitter( aes( color = method), size = 0.01) +
  facet_wrap( ~ label, ncol = 3) 

print( plotobj)

outfilename = "test06R"

ggsave( file = paste( outfilename, "02.png", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)
ggsave( file = paste( outfilename, "02.svg", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)


## 以下ではC++/Boostを基準にするので、基準を外したdfを使う
df_long2 = df_long[ df_long$method != "C++/Boost", ]


## グラフで差分を確認

plotobj = ggplot( df_long2, aes( x = p, y = diff)) +
  labs( x = "p") +
  labs( y = "Diff") +
  labs( title = "Difference against C++/Boost; Beta InvCDF") +
  theme( plot.title = element_text( hjust = 0.5)) +
  geom_point( aes( color = method), size = 1) +
  facet_wrap( ~ label, ncol = 3) 

print( plotobj)

outfilename = "test06R"

ggsave( file = paste( outfilename, "03.png", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)
ggsave( file = paste( outfilename, "03.svg", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)

# ジッタリングする

plotobj = ggplot( df_long2, aes( x = p, y = diff)) +
  labs( x = "p") +
  labs( y = "Diff") +
  theme( plot.title = element_text( hjust = 0.5)) +
  geom_jitter( aes( color = method), size = 0.1) +
  facet_wrap( ~ label, ncol = 3) 

print( plotobj)

outfilename = "test06R"

ggsave( file = paste( outfilename, "04.png", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)
ggsave( file = paste( outfilename, "04.svg", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)


## グラフで相対誤差を確認、statrだけ

df_long3 = df_long2[ df_long2$method == "Rust/statr", ]
df_long3$log10relerr = log10( df_long3$relerr)

plotobj = ggplot( df_long3, aes( x = p, y = log10relerr)) +
  labs( x = "p") +
  labs( y = "Log10( Relative Error)") +
  labs( title = "Relative Error; Rust/statrs's Beta InvCDF") +
  theme( plot.title = element_text( hjust = 0.5)) +
  geom_point( size = 1) +
  facet_wrap( ~ label, ncol = 3)

print( plotobj)

outfilename = "test06R"

ggsave( file = paste( outfilename, "05.png", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)
ggsave( file = paste( outfilename, "05.svg", sep = ""), plot = plotobj, dpi = 300, width = 6, height = 6)

