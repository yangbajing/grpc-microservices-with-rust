"use client";
import { useEffect, useState } from "react";
import { userClient } from "@/lib/grpc-web";
import { Label } from "@/components/ui/label";
import { UserDto } from "@/pb/getting/v1/user";
import { isAbortError } from "abort-controller-x";

export default function Profile() {
  const [user, setUser] = useState<UserDto>();

  useEffect(() => {
    const abortController = new AbortController();
    userClient
      .get({ id: 1 }, { signal: abortController.signal })
      .then((res) => {
        console.log(res);
        setUser(res);
      })
      .catch((error) => {
        if (isAbortError(error)) {
          return; // 忽略 aboted 错误
        }
        throw error;
      });

    // 当 effect 退出时 自动取消请求异步请求
    return () => abortController.abort("manual aborted");
  }, []);

  useEffect(() => {
    const abortController = new AbortController();
    async function streams() {
      for await (const res of userClient.streamList({}, { signal: abortController.signal })) {
        console.log("streamList response:", res);
      }
    }

    streams().catch((error) => {
      if (isAbortError(error)) {
        return;
      }
      throw error;
    });

    return () => abortController.abort("manual aborted");
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
