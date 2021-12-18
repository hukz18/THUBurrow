import type { NextPage, GetStaticProps } from 'next';
import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/router';
import {
  Layout,
  Breadcrumb,
  message,
  Card,
  PageHeader,
  Button,
  Menu,
  Tag,
} from 'antd';
import PostList from '../components/post-list';
import GlobalHeader from '../components/header/header';
import '../node_modules/antd/dist/antd.css';
import axios, { AxiosError } from 'axios';

const operationTabList = [
  {
    key: '',
    tab: <span>全部</span>,
  },
  {
    key: '学习',
    tab: <span>学习</span>,
  },
  {
    key: '生活',
    tab: <span>生活</span>,
  },
];

const { Header, Content, Footer } = Layout;

axios.defaults.withCredentials = true;
axios.defaults.headers.post['Content-Type'] = 'application/json';

const Home: NextPage = () => {
  const router = useRouter();
  const [menuMode, setMenuMode] = useState<'inline' | 'horizontal'>(
    'horizontal'
  );
  const [postList, setPostList] = useState([]);
  const [page, setPage] = useState(1);
  const [section, setsection] = useState('');
  useEffect(() => {
    const fetchPostList = async () => {
      try {
        var url;
        if (section == '') {
          url = `${process.env.NEXT_PUBLIC_BASEURL}/content/list?page=${
            page - 1
          }`;
        } else {
          url = `${process.env.NEXT_PUBLIC_BASEURL}/content/list?page=${
            page - 1
          }&section=${section}`;
        }
        const res = await axios.get(url, {
          headers: { 'Content-Type': 'application/json' },
        });
        const postlist = res.data.list_page.post_page;
        setPostList(postlist);
      } catch (error) {
        const err = error as AxiosError;
        if (err.response?.status == 401) {
          message.error('请先登录！');
          router.push('/login');
        }
      }
    };
    fetchPostList();
  }, [page, router, section]);

  const changesection = (value: string) => {
    setsection(value);
  };

  return (
    <Layout className='layout'>
      <Header>
        <title>首页</title>
        <GlobalHeader />
      </Header>
      <Content style={{ padding: '0 50px' }}>
        <Card
          style={{ margin: '16px 0' }}
          bordered={false}
          tabList={operationTabList}
          activeTabKey={section}
          onTabChange={changesection}
        >
          <Card>
            <PostList listData={postList} setPage={setPage} />
          </Card>
        </Card>
      </Content>
      <Footer style={{ textAlign: 'center' }}>THUBurrow © 2021</Footer>
    </Layout>
  );
};

export default Home;
