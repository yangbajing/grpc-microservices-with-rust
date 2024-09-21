import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { signin } from "@/actions/sign";

export default function Signin() {
  return (
    <div className="w-full h-full">
      <form action={signin} className="w-80 mx-auto mt-20 block justify-center items-center space-y-4">
        <div>
          <Label htmlFor="email">邮箱</Label>
          <Input type="email" id="email" name="email" />
        </div>
        <div>
          <Label htmlFor="password">密码</Label>
          <Input type="password" id="password" name="password" />
        </div>
        <div>
          <Button type="submit">登录</Button>
        </div>
      </form>
    </div>
  );
}
