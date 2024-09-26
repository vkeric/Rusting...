git add . 暂存

git commit -m "message" 提交信息

git pull 拉取

git push  推送

git branch   查看本地_分支

git branch -m new_name 重命名

git branch - r   查看远程分支

git branch - a    查看所有分支,包括本都和远程(remotes/开头表示远程分支)

git branch --merged 查看哪些分支已经合并到当前分支

git branch --no-merged 查看所有未合并工作的分支



git branch 新分支  创建新分支操作

git checkout 分支名称  切换分支操作

git checkout -b 新分支名称  创建分支且切换到新分支



 ### 合并分支

1. 切换到主分支
2. git pull origin 主分支名称
3. git merge 分支名称
4. get status 查看状态 
5. git push origin 主分支名称



### 删除分支

git branch -D 删除本地分支

git push origin :master 删除远程分支

