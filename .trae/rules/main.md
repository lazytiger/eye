1. 请记住这是一个在windows上开发的项目，所以任何脚本或者命令都需要能在windows下运行，不能使用linux下的命令。
2. 优先使用powershell指令，避免使用cmd指令。
3. android版本可以在本环境下尝试构建，但是iOS和osx版本都不能在windows上构建，避免尝试在windows上构建这两个版本。
4. 使用rust的crate时请先去crates.io上查看最新版本，避免使用旧版本的crate。如果crate的源码中带有example则优先查看example的用法并根据example的用法进行调用。
5. 不要自动commit修改，commit前一定要向用户确认。
6. 一定要有测试代码，测试代码要覆盖所有功能，测试代码放到tests目录下，与源代码分开。
7. 生成的代码要有清晰但是不啰嗦的注释，注释要能准确描述代码的功能，注释请使用英文。
8. 每次生成完代码之后都要保证能正常编译通过。