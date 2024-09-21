"use server";
import { SigninResponse, TokenType } from "@/pb/getting/v1/auth";
import { authClient } from "@/lib/grpc";

export async function signin(formData: FormData) {
  const email = formData.get("email") as string;
  const password = formData.get("password") as string;

  const request = { email, password };
  try {
    const response: SigninResponse = await authClient.signin(request);

    console.log("登录成功:", response);
    console.log("令牌类型:", TokenType[response.tokenType]);
  } catch (error) {
    console.error("登录失败:", error);
  }
}
