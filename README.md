![header.png](https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fpic3.zhimg.com%2Fv2-a2e2c4bb877d624b7897d23c72bf58e2_1440w.jpg%3Fsource%3D172ae18b&refer=http%3A%2F%2Fpic3.zhimg.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=jpeg?sec=1645062176&t=ce1db215e1f5fc0c05542fed304ea999)

# 基于有道翻译API的中英翻译软件

# 简介
>为解决编程以及打开翻译网站来回切换的繁琐，借此机会开发基于有道翻译API的linux翻译软件。

# 安装和使用
```  
方案一:
    1. git clone git@github.com:edison-cl/youdao.git
    2. cargo build --release (可能需要安装某些依赖,例如openssl,pkg-config)
    3. ./youdao {你想翻译的内容, 中英文都可以}

```
> 第一次使用后程序会自动添加到软链接, 往后使用只需输入youdao即可
> 本项目未适配window!


![snipaste.png](https://github.com/edison-cl/youdao/blob/master/Snipaste.jpg?raw=true)