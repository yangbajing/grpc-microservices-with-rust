"use client";
import { useEffect, useState } from "react";
import { userClient } from "@/lib/grpc-web";
import { Label } from "@/components/ui/label";
import { UserDto } from "@/pb/getting/v1/user";

export default function Profile() {
  const [user, setUser] = useState<UserDto>();
  useEffect(() => {
    userClient.get({ id: 1 }).then((res) => {
      console.log(res);
      setUser(res);
    });
  }, []);

  return (
    <div className="w-full h-full">
      <form className="w-80 mx-auto mt-20 block justify-center items-center space-y-4">
        <div>
          <Label>名字</Label>
          <p>{user?.name}</p>
        </div>
        <div>
          <Label>邮箱</Label>
          <p>{user?.email}</p>
        </div>
        <div>
          <Label>状态</Label>
          <p>{user?.status}</p>
        </div>
      </form>
    </div>
  );
}
