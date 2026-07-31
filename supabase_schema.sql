-- =========================================================================
-- Justino Studio: Supabase Database Schema
-- Execute este script no SQL Editor do seu projeto Supabase.
-- =========================================================================

-- 1. Profiles (Dados do Desenvolvedor)
CREATE TABLE public.profiles (
  id uuid REFERENCES auth.users ON DELETE CASCADE PRIMARY KEY,
  username text UNIQUE NOT NULL,
  avatar_url text,
  theme_preference text DEFAULT 'justino-volcanic',
  language text DEFAULT 'en',
  created_at timestamp with time zone DEFAULT timezone('utc'::text, now()) NOT NULL
);

-- Habilitar RLS (Segurança de Linha)
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Public profiles are viewable by everyone." ON public.profiles FOR SELECT USING (true);
CREATE POLICY "Users can insert their own profile." ON public.profiles FOR INSERT WITH CHECK (auth.uid() = id);
CREATE POLICY "Users can update own profile." ON public.profiles FOR UPDATE USING (auth.uid() = id);

-- 2. Workspaces (Projetos em Nuvem)
CREATE TABLE public.workspaces (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  owner_id uuid REFERENCES public.profiles(id) ON DELETE CASCADE NOT NULL,
  name text NOT NULL,
  description text,
  is_public boolean DEFAULT false,
  created_at timestamp with time zone DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamp with time zone DEFAULT timezone('utc'::text, now()) NOT NULL
);

ALTER TABLE public.workspaces ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can view their own workspaces" ON public.workspaces FOR SELECT USING (auth.uid() = owner_id OR is_public = true);
CREATE POLICY "Users can create workspaces" ON public.workspaces FOR INSERT WITH CHECK (auth.uid() = owner_id);
CREATE POLICY "Users can update their own workspaces" ON public.workspaces FOR UPDATE USING (auth.uid() = owner_id);
CREATE POLICY "Users can delete their own workspaces" ON public.workspaces FOR DELETE USING (auth.uid() = owner_id);

-- 3. Cloud Files (Arquivos .jucode na Nuvem)
CREATE TABLE public.cloud_files (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  workspace_id uuid REFERENCES public.workspaces(id) ON DELETE CASCADE NOT NULL,
  file_path text NOT NULL,
  content text DEFAULT '',
  created_at timestamp with time zone DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamp with time zone DEFAULT timezone('utc'::text, now()) NOT NULL,
  UNIQUE(workspace_id, file_path)
);

ALTER TABLE public.cloud_files ENABLE ROW LEVEL SECURITY;
-- A segurança herda do Workspace (Simplificando: só dono edita)
CREATE POLICY "Users can view files of accessible workspaces" ON public.cloud_files FOR SELECT USING (
  EXISTS (SELECT 1 FROM public.workspaces WHERE id = cloud_files.workspace_id AND (owner_id = auth.uid() OR is_public = true))
);
CREATE POLICY "Users can insert/update files in their workspaces" ON public.cloud_files FOR ALL USING (
  EXISTS (SELECT 1 FROM public.workspaces WHERE id = cloud_files.workspace_id AND owner_id = auth.uid())
);

-- 4. Marketplace Extensions
CREATE TABLE public.marketplace_extensions (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  author_id uuid REFERENCES public.profiles(id) ON DELETE SET NULL,
  name text NOT NULL UNIQUE,
  description text,
  version text DEFAULT '1.0.0',
  type text NOT NULL CHECK (type IN ('theme', 'language_support', 'tool')),
  downloads integer DEFAULT 0,
  created_at timestamp with time zone DEFAULT timezone('utc'::text, now()) NOT NULL
);

ALTER TABLE public.marketplace_extensions ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Extensions are viewable by everyone" ON public.marketplace_extensions FOR SELECT USING (true);
CREATE POLICY "Authors can update their extensions" ON public.marketplace_extensions FOR UPDATE USING (auth.uid() = author_id);
CREATE POLICY "Authenticated users can publish extensions" ON public.marketplace_extensions FOR INSERT WITH CHECK (auth.uid() = author_id);

-- Trigger automatizado: Cria um 'profile' automaticamente quando o usuário faz Sign Up no Supabase Auth
CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS trigger AS $$
BEGIN
  INSERT INTO public.profiles (id, username, avatar_url)
  VALUES (new.id, split_part(new.email, '@', 1), new.raw_user_meta_data->>'avatar_url');
  RETURN new;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE TRIGGER on_auth_user_created
  AFTER INSERT ON auth.users
  FOR EACH ROW EXECUTE PROCEDURE public.handle_new_user();
